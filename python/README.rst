Snips NLU utils Python wrapper
==============================

.. image:: https://travis-ci.org/snipsco/snips-nlu-utils.svg?branch=master
    :target: https://travis-ci.org/snipsco/snips-nlu-utils

.. image:: https://img.shields.io/pypi/v/snips-nlu-utils.svg?branch=master
    :target: https://pypi.python.org/pypi/snips-nlu-utils

.. image:: https://img.shields.io/pypi/pyversions/snips-nlu-utils.svg?branch=master
    :target: https://pypi.python.org/pypi/snips-nlu-utils


This library is a wrapper of a Rust NLU utils library, which is used by Snips NLU

Installation
------------

-----------
Linux / OSX
-----------

On linux and OSX, you can install this package easily using pip:

.. code-block:: console

    pip install snips-nlu-utils


---------------
Other platforms
---------------

For the other platforms, you will install the package from a source distribution
containing some ``rust`` code, hence ``rust`` must be installed on your machine:

To install Rust, run the following in your terminal, then follow the onscreen instructions:

.. code-block:: console

    curl https://sh.rustup.rs -sSf | sh


You will also need the python lib ``setuptools_rust``:

.. code-block:: console

    pip install setuptools_rust

Finally, you can install ``snips-nlu-utils`` using pip as before:

.. code-block:: console

    pip install snips-nlu-utils



