"""Entry point for the OutOf5K demo parser service."""

from .main import ServiceMain


def main() -> None:
    """Run the demo parser service."""
    service = ServiceMain()
    service.run()


if __name__ == "__main__":
    main()
