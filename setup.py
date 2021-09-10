from setuptools import setup
from setuptools_rust import Binding, RustExtension

with open("README.md", "r", encoding="utf-8") as fh:
    long_description = fh.read()

setup(
    name="plrs",
    version="0.1.3",
    author="Jake Roggenbuck",
    description="The multi-tool of lexical analysis and tokenization.",
    author_email="me@jr0.org",
    long_description=long_description,
    long_description_content_type="text/markdown",
    url="https://github.com/JakeRoggenbuck/plrs",
    rust_extensions=[
        RustExtension("plrs", binding=Binding.PyO3),
    ],
    packages=["plrs"],
    zip_safe=False,
)
