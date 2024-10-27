from setuptools import setup, find_packages

setup(
    name="ETLpipelineWenyeLi",
    version="0.1.0",
    description="ETLpipline",
    author="Wenye Li",
    author_email="wl275@duke.edu",
    packages=find_packages(),
    install_requires=[
        "databricks-sql-connector",
        "pandas",
        "python-dotenv",
    ],
    entry_points={
        "console_scripts": [
            "etl=main:main",
        ],
    },
)
