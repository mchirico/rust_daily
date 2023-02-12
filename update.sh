git checkout --orphan temp_branch
git add .gitignore
git add ./day1/.gitignore
git add -A

git commit -am "Initial commit"

git branch -D main
git branch -m main

git push -f origin main
