git msg="up":
    -find . -name .DS_Store | xargs rm -rf {};
    -cargo fmt;
    -git add .gitignore
    -git add .github/workflows/*.yml
    -git add **
    -git commit -m "{{msg}}"
    -git push


build:
    #bunx tailwindcss -i ./style/input.css -o ./css/main.css --minify
    trunk build --release --config Trunk.toml;

r:
    trunk build --release --config Trunk.toml;
    miniserve ./dist -p 10404 -i 127.0.0.1 --spa --index index.html;


deploy:
    trunk build --release --config Trunk.toml;
    bunx wrangler deploy;
