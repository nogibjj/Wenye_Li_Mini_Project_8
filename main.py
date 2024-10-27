"""CLI script to run the ETL process"""
import sys
import argparse
from mylib.extract import extract
from mylib.transform_load import load
from mylib.query import (
    run_query,
)


def handle_arguments(args):
    """add action based on inital calls"""
    parser = argparse.ArgumentParser(description="ETL-Query script")
    parser.add_argument(
        "action",
        choices=[
            "extract",
            "transform_load",
            "run_query",
        ],
        # shows how to run output
        help="Action to perform (extract, transform_load, run_query)." + "\n\n",
    )
    args = parser.parse_args(args[:1])
    if args.action == "run_query":
        parser.add_argument("query")

    # parse again with ever
    return parser.parse_args(sys.argv[1:])


def main():
    """handles all the cli commands"""
    args = handle_arguments(sys.argv[1:])

    if args.action == "extract":
        print("Extracting data...")
        extract()
    elif args.action == "transform_load":
        print("Transforming data...")
        load()
    elif args.action == "run_query":
        run_query(args.query)
    else:
        print("Invalid action. Please provide a valid action.")


if __name__ == "__main__":
    main()
