# INTERLOCK NETWORK - PYTHON CONTRACT SHELL
#
# This is a shell / boilerplate to enable backend python scripts to call
# and manipulate Aleph Zero smart contracts via the Polkadot.js script libraries.
# (ilockmvp.js, ilockaccess.js, etc)
#

import js2py

result_access, tempfile = js2py.run_file("ilockaccess.js");
# result_mvp, javascript_mvp = js2py.run_file("ilockmvp.js");

output = tempfile.helloWorld();

print(output);
