from aodict import AODict


def test_aodict():
    ao = AODict()
    ao["a"] = 1
    assert ao["a"] == 1

    ao[1] = "a"
    assert ao[1] == "a"


def test_aodict_does_not_update_values():
    ao = AODict()
    ao["a"] = 1
    assert ao["a"] == 1
    ao["a"] = 2
    assert ao["a"] == 1


def test_iterator():
    ao = AODict()

    keys = ["a", "1", "2", "3", "d"]
    for i, k in enumerate(keys):
        ao[k] = i

    for k in ao:
        assert k in keys
        assert k in ao
