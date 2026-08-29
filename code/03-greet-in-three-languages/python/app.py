from wit_world.imports import environment


# componentize-py wires this up by name: the generated `wit_world.exports`
# module expects an attribute matching each exported WIT interface (here,
# `Run`, from `wasi:cli/run`), whose method implements that interface's
# function.
class Run:
    def run(self) -> None:
        args = environment.get_arguments()
        # args[0] is argv[0] (WASI hands back the program name there, same
        # POSIX convention as C) - the first real CLI argument is args[1].
        name = args[1] if len(args) > 1 else "world"
        print(f"Hello, {name}, from Python!")
