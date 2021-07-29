from setuptools import setup
from setuptools_rust import Binding, RustExtension

setup(
    name="plrs",
    version="1.0",
    rust_extensions=[
        RustExtension("plrs_tokenizer", binding=Binding.PyO3),
        RustExtension("plrs_lexer", binding=Binding.PyO3),
    ],
    py_modules=["plrs_tokenizer", "plrs_lexer"],
    zip_safe=False,
)
