from sys import version_info
from subprocess import run, CalledProcessError
from shutil import copyfile, copytree, rmtree

v = version_info

if v[0] < 3 or (v[0] == 3 and v[1] < 8):
    print("Python 3.8 or a more recent version is required.")
    exit()

print("BUILD: remove 'build/' directory if exists")
rmtree("build/", ignore_errors=True)

wasm_args = ["wasm-pack", "build", "--target", "web"]
print(f"BUILD: compiling with '{' '.join(wasm_args)}'")
build = run(wasm_args)
build.check_returncode()

print("BUILD: inserting files to 'build/'")
copytree("static/", "build/", dirs_exist_ok=True)
copyfile("pkg/portfolio.js", "build/portfolio.js")
copyfile("pkg/portfolio_bg.wasm", "build/portfolio_bg.wasm")

less_args = ["npx", "lessc", "less/index.less", "build/css/index.css"]
print("BUILD: compiling CSS")
build = run(less_args, shell=True)
build.check_returncode()

print("BUILD: completed")