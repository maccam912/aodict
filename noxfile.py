from nox import session

python_versions = ["3.10", "3.9", "3.8", "3.7"]


@session(python=python_versions[0])
def black(session):
    session.install("black")
    session.run("black", ".")


@session
def cargo_fmt(session):
    session.run("cargo", "fmt", external=True)


@session(python=python_versions)
def test(session):
    session.install("pytest", ".")
    session.run("pytest")