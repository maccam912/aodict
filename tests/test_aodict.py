from aodict import AODict

def test_aodict():
    ao = AODict()

    ao["a"] = 1
    assert ao["a"] == 1

    ao[1] = "a"
    assert ao[1] == "a"

    ao["a"] = 2
    assert ao["a"] == 1

