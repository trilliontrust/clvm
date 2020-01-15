# make the "eval" function for a vm with the given operators

import os

from .EvalError import EvalError


def make_eval_f(operator_lookup, quote_kw, args_kw):

    CLVM_DISALLOW_E_OP = os.environ.get('CLVM_DISALLOW_E_OP')

    def eval_core(eval_f, form, env):
        if not form.listp():
            raise EvalError("not a list", form)

        if form.nullp():
            raise EvalError("eval cannot handle empty list", form)

        first_item = form.first()

        if first_item.listp():
            new_form = eval_f(eval_f, first_item, env)
            return eval_f(eval_f, new_form.first(), new_form.rest())

        f_index = first_item.as_atom()
        if f_index is None:
            raise EvalError("non-byte atom in first element of list", form)

        # special form QUOTE

        if f_index == quote_kw:
            if form.rest().nullp() or not form.rest().rest().nullp():
                raise EvalError("quote requires exactly 1 parameter", form)
            return form.rest().first()

        # TODO: rewrite with cons, rest, etc.
        args = form.to([eval_f(eval_f, _, env) for _ in form.rest().as_iter()])

        # keyword ENV

        if f_index == args_kw:
            if form.nullp() or not form.rest().nullp():
                raise EvalError("env requires no parameters", form)
            return env

        # special form APPLY

        f = operator_lookup.get(f_index)
        if f:
            return f(args)

        raise EvalError("unimplemented operator", first_item)

    return eval_core
