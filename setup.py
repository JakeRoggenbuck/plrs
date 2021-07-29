from setuptools import setup
from setuptools_rust import Binding, RustExtension

setup(
    name="plrs",
    version="1.0",
    rust_extensions=[RustExtension("plrs_tokenizer", binding=Binding.PyO3)],
    py_modules=["plrs_tokenizer"],
    zip_safe=False,
)
