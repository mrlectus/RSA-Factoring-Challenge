from setuptools import setup
from Cython.Build import cythonize

setup(
    name='find factors',
    ext_modules=cythonize("factors.pyx"),
    zip_safe=False,
)
