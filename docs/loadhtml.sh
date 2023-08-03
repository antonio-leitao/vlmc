markdown_content=$(cat DOCME.md)

gh api \
  --method POST \
  -H "Accept: application/vnd.github+json" \
  -H "X-GitHub-Api-Version: 2022-11-28" \
  /markdown \
  -f text="$markdown_content" > DOCME.html
