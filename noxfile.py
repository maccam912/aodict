from nox import session

@session(python=["3.10", "3.9", "3.8", "3.7"])
def test(session):
    session.install("pytest", ".")
    session.run("pytest")