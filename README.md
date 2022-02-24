Hi all,

This is a simple POC to load and execute a [Chaos Toolkit][ctk] experiment from
a binary implemented in Rust.

[ctk]: https://chaostoolkit.org

First make sure you have [installed Chaos Toolkit][ctkinstall]
as usual in a virtual environment.

[ctkinstall]: https://chaostoolkit.org/reference/usage/install/

```
$ python3 -m venv .venv
$ source .venv/bin/activate
$ pip install -U chaostoolkit
```

Now build the binary:

```
$ cargo build
```

You can now run it:

```
$ ./target/debug/ctkrust
```

This should load the experiment and run it using Chaos Toolkit libraries
imported by Rust at runtime.