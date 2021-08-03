from setuptools import setup
from setuptools_rust import Binding, RustExtension

setup(
    name="libalpha",
    version="0.0.1",
    license="MIT",
    rust_extensions=[
        RustExtension("libalpha.libalpha", binding=Binding.PyO3, debug=False)
    ],
    package_dir={"": "py_src"},
    packages=[
        "libalpha",
        "libalpha.utils",
    ],
    package_data={
        "libalpha": ["py.typed", "__init__.pyi"],
        "libalpha.utils": ["py.typed", "__init__.pyi"],
    },
    zip_safe=False,
)