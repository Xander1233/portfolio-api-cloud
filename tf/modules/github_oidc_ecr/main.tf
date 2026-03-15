locals {
  branch_subjects = [for branch in var.github_oidc_branches : "repo:${var.github_repository}:ref:refs/heads/${branch}"]
  trusted_subjects = concat(
    local.branch_subjects,
    [
      # release events and git-tag pushes
      "repo:${var.github_repository}:ref:refs/tags/*",
      # workflow_dispatch without an explicit ref (GitHub sets this for default-branch dispatches)
      "repo:${var.github_repository}:workflow_dispatch",
    ]
  )
}

data "aws_iam_openid_connect_provider" "github" {
  url = "https://token.actions.githubusercontent.com"
}

data "aws_iam_policy_document" "trust" {
  statement {
    actions = ["sts:AssumeRoleWithWebIdentity"]

    principals {
      type        = "Federated"
      identifiers = [data.aws_iam_openid_connect_provider.github.arn]
    }

    condition {
      test     = "StringEquals"
      variable = "token.actions.githubusercontent.com:aud"
      values   = ["sts.amazonaws.com"]
    }

    condition {
      test     = "StringLike"
      variable = "token.actions.githubusercontent.com:sub"
      values   = local.trusted_subjects
    }
  }
}

data "aws_iam_policy_document" "ecr_push" {
  statement {
    sid = "GetAuthorizationToken"
    actions = [
      "ecr:GetAuthorizationToken"
    ]
    resources = ["*"]
  }

  statement {
    sid = "PushToHealthLambdaRepository"
    actions = [
      "ecr:BatchCheckLayerAvailability",
      "ecr:BatchGetImage",
      "ecr:CompleteLayerUpload",
      "ecr:DescribeRepositories",
      "ecr:InitiateLayerUpload",
      "ecr:PutImage",
      "ecr:UploadLayerPart"
    ]
    resources = [var.ecr_repository_arn]
  }
}

resource "aws_iam_role" "github_ecr_push" {
  name               = "${var.name_prefix}-github-ecr-push"
  assume_role_policy = data.aws_iam_policy_document.trust.json

  tags = merge(var.tags, {
    Name = "${var.name_prefix}-github-ecr-push"
  })
}

resource "aws_iam_policy" "github_ecr_push" {
  name   = "${var.name_prefix}-github-ecr-push"
  policy = data.aws_iam_policy_document.ecr_push.json

  tags = merge(var.tags, {
    Name = "${var.name_prefix}-github-ecr-push"
  })
}

resource "aws_iam_role_policy_attachment" "github_ecr_push" {
  role       = aws_iam_role.github_ecr_push.name
  policy_arn = aws_iam_policy.github_ecr_push.arn
}
