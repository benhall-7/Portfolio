from sys import version_info
from subprocess import run, CalledProcessError
from shutil import copyfile, copytree, rmtree

v = version_info

if v[0] < 3 or (v[0] == 3 and v[1] < 8):
    print("Python 3.8 or a more recent version is required.")
    exit()

args = ["wasm-pack", "build", "--target", "web"]
print(f"BUILD: compiling with '{' '.join(args)}'")
build = run(args)
build.check_returncode()

print("BUILD: remove 'build/' directory if exists")
rmtree("build/", ignore_errors=True)
print("BUILD: inserting files to 'build/'")
copytree("static/", "build/", dirs_exist_ok=True)
copyfile("pkg/portfolio.js", "build/portfolio.js")
copyfile("pkg/portfolio_bg.wasm", "build/portfolio_bg.wasm")
print("BUILD: completed")
