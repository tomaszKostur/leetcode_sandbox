from collections.abc import Sequence
from typing import TypeAlias

Interval: TypeAlias = tuple[int, int]


def find_repetition_intervals(elements: Sequence[int]) -> dict[int, Interval]:
    """Return the first and last index of every repeated value."""
    first_seen: dict[int, int] = {}
    intervals: dict[int, Interval] = {}

    for index, value in enumerate(elements):
        if value not in first_seen:
            first_seen[value] = index
        else:
            intervals[value] = (first_seen[value], index)

    return intervals


def remove_contained_intervals(
    intervals: dict[int, Interval],
) -> dict[int, Interval]:
    """Remove intervals fully contained within another interval."""
    result: dict[int, Interval] = {}

    for value, interval in intervals.items():
        start, end = interval

        is_contained = any(
            other_value != value
            and other_start <= start
            and end <= other_end
            and (other_start < start or end < other_end)
            for other_value, (other_start, other_end) in intervals.items()
        )

        if not is_contained:
            result[value] = interval

    return result


def interval_sum(
    prefix_sums: Sequence[int],
    start: int,
    end: int,
) -> int:
    """Return the sum of an inclusive interval."""
    return prefix_sums[end + 1] - prefix_sums[start]


def best_sum(elements: Sequence[int]) -> tuple[int, Interval] | None:
    """Find the non-contained repetition interval with the highest sum."""
    intervals = remove_contained_intervals(
        find_repetition_intervals(elements)
    )

    if not intervals:
        return None

    prefix_sums = [0]
    for value in elements:
        prefix_sums.append(prefix_sums[-1] + value)

    _, best_interval = max(
        intervals.items(),
        key=lambda item: interval_sum(
            prefix_sums,
            item[1][0],
            item[1][1],
        ),
    )

    return (
        interval_sum(prefix_sums, *best_interval),
        best_interval,
    )


def test_split_collections() -> None:
    data = [2, 3, 9, 9, 9, 3, 4, 999, 4]
    assert best_sum(data) == (1007, (6, 8))


def test_overlapping_collection() -> None:
    data = [2, 3, 9, 9, 9, 3, 999, 1, 9, 4]
    assert best_sum(data) == (1033, (2, 8))


def test_no_repetitions() -> None:
    assert best_sum([1, 2, 3]) is None


def test_negative_values() -> None:
    assert best_sum([-5, 1, -5]) == (-9, (0, 2))


if __name__ == "__main__":
    test_split_collections()
    test_overlapping_collection()
    test_no_repetitions()
    test_negative_values()