from setuptools import find_packages, setup

setup(
    name = 'rusthypergraph',
    packages = find_packages(include=['wrapper']),
    version = '0.1.1',
    description = '...',
    author = 'Giovanni',
    install_requires = ['Hypergraphx', 'rusthypergraph'],
    #setup_requires = ['pytest_runner'],
    #test_reqire = ['pytest'],
    #test_suite'tests',
)