#!/usr/bin/env bash
set -euo pipefail

exec > >(tee -a /var/log/user-data.log) 2>&1
echo "[user-data] start $(date -Is)"

IMAGE_URI="${image_uri}"
CONTAINER_NAME="${container_name}"
HOST_PORT="${host_port}"
CONTAINER_PORT="${container_port}"

S3_CONFIG_URI="${s3_config_uri}"
CONTAINER_CONFIG_PATH="${container_config_mount_path}"
LOGS_DIR="${logs_dir}"
LOG_GROUP_NAME="${log_group_name}"

AWS_REGION="${aws_region}"
EXTRA_DOCKER_ARGS="${extra_docker_args}"

APP_DIR="/etc/$CONTAINER_NAME"
CONFIG_FILE_HOST="$APP_DIR/config.toml"

mkdir -p "$APP_DIR"
mkdir -p "$LOGS_DIR"
chmod 700 "$APP_DIR"

export AWS_REGION AWS_DEFAULT_REGION="$AWS_REGION"

apt-get update -y
apt-get install -y ca-certificates curl gnupg lsb-release jq unzip

install -m 0755 -d /etc/apt/keyrings
if [[ ! -f /etc/apt/keyrings/docker.gpg ]]; then
  curl -fsSL https://download.docker.com/linux/ubuntu/gpg | gpg --dearmor -o /etc/apt/keyrings/docker.gpg
  chmod a+r /etc/apt/keyrings/docker.gpg
fi

UBUNTU_CODENAME="$(. /etc/os-release && echo "$VERSION_CODENAME")"
echo "deb [arch=$(dpkg --print-architecture) signed-by=/etc/apt/keyrings/docker.gpg] https://download.docker.com/linux/ubuntu $UBUNTU_CODENAME stable" > /etc/apt/sources.list.d/docker.list

apt-get update -y
apt-get install -y docker-ce docker-ce-cli containerd.io docker-buildx-plugin docker-compose-plugin
systemctl enable --now docker

if ! command -v aws >/dev/null 2>&1 || ! aws --version 2>/dev/null | grep -q "aws-cli/2"; then
  tmpdir="$(mktemp -d)"
  curl -fsSL "https://awscli.amazonaws.com/awscli-exe-linux-x86_64.zip" -o "$tmpdir/awscliv2.zip"
  unzip -q "$tmpdir/awscliv2.zip" -d "$tmpdir"
  "$tmpdir/aws/install" --update
  rm -rf "$tmpdir"
fi

CLOUDWATCH_AGENT_URL="https://amazoncloudwatch-agent-$AWS_REGION.s3.$AWS_REGION.amazonaws.com/ubuntu/amd64/latest/amazon-cloudwatch-agent.deb"
curl -fsSL "$CLOUDWATCH_AGENT_URL" -o /tmp/amazon-cloudwatch-agent.deb
dpkg -i /tmp/amazon-cloudwatch-agent.deb
rm -f /tmp/amazon-cloudwatch-agent.deb

cat > /opt/aws/amazon-cloudwatch-agent/etc/amazon-cloudwatch-agent.json <<EOF
{
  "logs": {
    "logs_collected": {
      "files": {
        "collect_list": [
          {
            "file_path": "$LOGS_DIR/*.log.*",
            "log_group_name": "$LOG_GROUP_NAME",
            "log_stream_name": "{instance_id}-app",
            "retention_in_days": 14
          }
        ]
      }
    }
  }
}
EOF

/opt/aws/amazon-cloudwatch-agent/bin/amazon-cloudwatch-agent-ctl -a fetch-config -m ec2 -s -c file:/opt/aws/amazon-cloudwatch-agent/etc/amazon-cloudwatch-agent.json

aws s3 cp "$S3_CONFIG_URI" "$CONFIG_FILE_HOST"
chmod 600 "$CONFIG_FILE_HOST"

docker pull "$IMAGE_URI"

SERVICE_FILE="/etc/systemd/system/$CONTAINER_NAME.service"
cat > "$SERVICE_FILE" <<EOF
[Unit]
Description=Docker Container - $CONTAINER_NAME
After=network-online.target docker.service
Wants=network-online.target

[Service]
Type=simple
Environment=AWS_REGION=$AWS_REGION
Environment=AWS_DEFAULT_REGION=$AWS_REGION
ExecStartPre=/usr/bin/docker pull $IMAGE_URI
ExecStartPre=-/usr/bin/docker rm -f $CONTAINER_NAME
ExecStart=/usr/bin/docker run --name $CONTAINER_NAME \\
  -p $HOST_PORT:$CONTAINER_PORT \\
  -e AWS_REGION=$AWS_REGION \\
  -e AWS_DEFAULT_REGION=$AWS_REGION \\
  -e RUST_LOG=info \\
  -v $CONFIG_FILE_HOST:$CONTAINER_CONFIG_PATH:ro \\
  -v $LOGS_DIR:/var/log/$CONTAINER_NAME/logs \\
  $EXTRA_DOCKER_ARGS \\
  $IMAGE_URI
ExecStop=/usr/bin/docker stop $CONTAINER_NAME
Restart=always
RestartSec=5s
TimeoutStartSec=0

[Install]
WantedBy=multi-user.target
EOF

systemctl daemon-reload
systemctl enable --now "$CONTAINER_NAME.service"

echo "[user-data] done $(date -Is)"
