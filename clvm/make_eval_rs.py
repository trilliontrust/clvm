# make the "eval" function for a vm with the given operators

import io

from csr import do_eval

from .serialize import sexp_from_stream, sexp_to_stream
from .EvalError import EvalError


def sexp_to_blob(sexp):
    f = io.BytesIO()
    sexp_to_stream(sexp, f)
    return f.getvalue()


def sexp_from_blob(blob):
    from .runtime_001 import to_sexp_f
    f = io.BytesIO(blob)
    return sexp_from_stream(f, to_sexp_f)


def make_eval_f(operator_lookup, quote_kw, eval_kw, env_kw):

    def eval_core(eval_f, form, env):
        form_blob = sexp_to_blob(env.first())
        env_blob = sexp_to_blob(env.rest())
        error, r_blob = do_eval(form_blob, env_blob)
        r = sexp_from_blob(bytes(r_blob))
        if error:
            raise EvalError(error, r)
        return r
    return eval_core
