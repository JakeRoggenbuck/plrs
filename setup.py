from setuptools import setup
from setuptools_rust import Binding, RustExtension

setup(
    name="plrs",
    version="1.0",
    rust_extensions=[
        RustExtension("plrs", binding=Binding.PyO3),
    ],
    py_modules=["plrs"],
    zip_safe=False,
)
