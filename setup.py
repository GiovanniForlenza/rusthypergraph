from setuptools import setup
from setuptools_rust import Binding, RustExtension

setup(
    name="rusthypergraph",
    version="0.1",
    rust_extensions=[RustExtension("rusthypergraph", binding=Binding.PyO3)],
    packages=["rusthypergraph"],
    zip_safe=False,
)
