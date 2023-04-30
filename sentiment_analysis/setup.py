from setuptools import find_packages, setup

setup(
    name="sent_analysis",
    version="0.1",
    packages=find_packages(exclude=["tests*"]),
    package_dir={"": "src"},
    license="MIT",
    description="A package for sentiment analysis",
    long_description=open("README.md").read(),
    install_requires=["transformers~=4.27.4", "duckduckgo-search~=2.8.6"],
)
