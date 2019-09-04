
chialisp reference


---------------------------------------------------------------


// building lists //


:::::::::


this builds a list of one item:  
(c (q ITEM) (q () ) )

e.g.  
'(c (q 123) (q () ) )'  
returns  
(123)


:::::::::


this builds a list of two items:  
(c (q ITEM1) (c (q ITEM2) (q () ) ) )

e.g.  
'(c (q 123) (c (q 456) (q () ) ) )'  
returns  
(123 456)


:::::::::


this builds a list of three items:  
(c (q ITEM1) (c (q ITEM2) (c (q ITEM3) (q () ) ) ) )

e.g.  
'(c (q 123) (c (q 456) (c (q 789) (q () ) ) ) )'  
returns  
(123 456 789)


:::::::::


this builds a list of two lists:  
(c LIST1 (c LIST2 (q () ) ) )

e.g.  
'(c (c (q 123) (c (q 456) (q () ) ) ) (c (c (q 123) (c (q 456) (c (q 789) (q () ) ) ) ) (q () ) ) )'  
returns  
((123 456) (123 456 789))


---------------------------------------------------------------


// using eval //


:::::::::


this returns the first item in the list:  
(e (f (a)) (a))

e.g.  
'(e (f (a)) (a))' '((q 123) (q 456) (q 789))'  
returns  
123


:::::::::


this returns the second item in the list:  
(e (f (r (a))) (a))

e.g.  
'(e (f (r (a))) (a))' '((q 123) (q 456) (q 789))'  
returns  
456


:::::::::


this returns the third item in a list:  
(e (f (r (r (a)))) (a))

e.g.  
'(e (f (r (r (a)))) (a))' '((q 123) (q 456) (q 789))'  
returns  
789


---------------------------------------------------------------


// manipulating items in a list //


:::::::::


this re-creates the list and returns it:  
'(e (f (a)) (a))' '((e (i (e (i (f (r (a))) (q (q ())) (q (q 1))) (a)) (q (q ())) (q (c (f (f (r (a)))) (e (f (a)) (c (f (a)) (c (r (f (r (a)))) (q ()))))))) (a)) (ITEM1 ITEM2 ... ITEMN))'

e.g.  
'(e (f (a)) (a))' '((e (i (e (i (f (r (a))) (q (q ())) (q (q 1))) (a)) (q (q ())) (q (c (f (f (r (a)))) (e (f (a)) (c (f (a)) (c (r (f (r (a)))) (q ()))))))) (a)) (123 456 789))'  
returns  
(123 456 789)


:::::::::


this takes a list and returns a list of lists, where the first inner list is the original list, the next inner list is the original list with the first item removed, the next inner list is the original list with the first two items removed, etc., and the last list contains only the last item in the original list:  
'(e (f (a)) (a))' '((e (i (e (i (f (r (a))) (q (q ())) (q (q 1))) (a)) (q (q ())) (q (c (f (r (a))) (e (f (a)) (c (f (a)) (c (r (f (r (a)))) (q ()))))))) (a)) (ITEM1 ITEM2 ... ITEMN))'

e.g.  
'(e (f (a)) (a))' '((e (i (e (i (f (r (a))) (q (q ())) (q (q 1))) (a)) (q (q ())) (q (c (f (r (a))) (e (f (a)) (c (f (a)) (c (r (f (r (a)))) (q ()))))))) (a)) (123 456 789))'  
returns  
((123 456 789) (456 789) (789))


:::::::::


this takes a list and returns a list of the squares of each item in the original list:  
'(e (f (a)) (a))' '((e (i (e (i (f (r (a))) (q (q ())) (q (q 1))) (a)) (q (q ())) (q (c (* (f (f (r (a)))) (f (f (r (a)))) ) (e (f (a)) (c (f (a)) (c (r (f (r (a)))) (q ()))))))) (a)) (ITEM1 ITEM2 ... ITEMN))'

e.g.  
'(e (f (a)) (a))' '((e (i (e (i (f (r (a))) (q (q ())) (q (q 1))) (a)) (q (q ())) (q (c (* (f (f (r (a)))) (f (f (r (a)))) ) (e (f (a)) (c (f (a)) (c (r (f (r (a)))) (q ()))))))) (a)) (123 456 789))'  
returns  
(15129 207936 622521)


:::::::::


this takes a list and returns a list of the sums of each item in the original list and some constant value:  
'(e (f (a)) (a))' '((e (i (e (i (f (r (a))) (q (q ())) (q (q 1))) (a)) (q (q ())) (q (c (+ (f (f (r (a)))) (q VALUETOADD) ) (e (f (a)) (c (f (a)) (c (r (f (r (a)))) (q ()))))))) (a)) (ITEM1 ITEM2 ... ITEMN))'

e.g.  
'(e (f (a)) (a))' '((e (i (e (i (f (r (a))) (q (q ())) (q (q 1))) (a)) (q (q ())) (q (c (+ (f (f (r (a)))) (q 500) ) (e (f (a)) (c (f (a)) (c (r (f (r (a)))) (q ()))))))) (a)) (123 456 789))'  
returns  
(623 956 1289)


---------------------------------------------------------------


// adding a prefix to items in a list //


:::::::::


this takes a list of items and returns a list of lists containing each item with some prefix:
wrapper:  
(e (q (e (f (a)) (a))) (c (q (PROGRAM)) (c (f (a)) (q ()))))  
program:  
(e (i (e (i (f (r (a))) (q (q ())) (q (q 1))) (a)) (q (q ())) (q (c (c (q PREFIX) (c (f (f (r (a)))) (q ()))) (e (f (a)) (c (f (a)) (c (r (f (r (a)))) (q ()))))))) (a))

e.g.  
'(e (q (e (f (a)) (a))) (c (q (e (i (e (i (f (r (a))) (q (q ())) (q (q 1))) (a)) (q (q ())) (q (c (c (q 33333) (c (f (f (r (a)))) (q ()))) (e (f (a)) (c (f (a)) (c (r (f (r (a)))) (q ()))))))) (a))) (c (f (a)) (q ()))))' '((123 456 789))'  
returns  
((33333 123) (33333 456) (33333 789))


:::::::::


this takes a list of lists and returns a list containing lists which each contain one item from the second list along with some prefix:  
wrapper:  
(e (q (e (f (a)) (a))) (c (q (PROGRAM)) (c (f (r (a))) (q ()))))  
program:  
(e (i (e (i (f (r (a))) (q (q ())) (q (q 1))) (a)) (q (q ())) (q (c (c (q PREFIX) (c (f (f (r (a)))) (q ()))) (e (f (a)) (c (f (a)) (c (r (f (r (a)))) (q ()))))))) (a))

e.g.  
'(e (q (e (f (a)) (a))) (c (q (e (i (e (i (f (r (a))) (q (q ())) (q (q 1))) (a)) (q (q ())) (q (c (c (q 33333) (c (f (f (r (a)))) (q ()))) (e (f (a)) (c (f (a)) (c (r (f (r (a)))) (q ()))))))) (a))) (c (f (r (a))) (q ()))))' '(("not what we want") (123 456 789))'  
returns  
((33333 123) (33333 456) (33333 789))


---------------------------------------------------------------


// checking lists for conditions //


:::::::::


this checks to see if the first item in a list is equal to some value; if it is, then it returns a list containing the second value in the list, and if it isn't, then it returns a list containing the third value in the list:  
'(e (i (= (f (a)) (q VALUETOCHECK) ) (q (c (f (r (a))) (q () ) )) (q (c (f (r (r (a)))) (q () ) ))) (a))' '(ITEM1 ITEM2 ITEM3)'

e.g.  
'(e (i (= (f (a)) (q 123) ) (q (c (f (r (a))) (q () ) )) (q (c (f (r (r (a)))) (q () ) ))) (a))' '(123 456 789)'  
returns  
(456)

e.g.  
'(e (i (= (f (a)) (q 123) ) (q (c (f (r (a))) (q () ) )) (q (c (f (r (r (a)))) (q () ) ))) (a))' '(023 456 789)'  
returns  
(789)


:::::::::


this checks to see if the second item in a list is equal to some value; if it is, then it returns a list containing the third value in the list, and if it isn't, then it returns a list containing a specified value:  
'(e (i (= (f (r (a))) (q VALUETOCHECK) ) (q (c (f (r (r (a)))) (q () ) )) (q (c (q SPECIFIEDVALUE) (q () ) ))) (a))' '(ITEM1 ITEM2 ITEM3)'

e.g.  
'(e (i (= (f (r (a))) (q 456) ) (q (c (f (r (r (a)))) (q () ) )) (q (c (q 99999) (q () ) ))) (a))' '(123 456 789)'  
returns  
(789)

e.g.  
'(e (i (= (f (r (a))) (q 456) ) (q (c (f (r (r (a)))) (q () ) )) (q (c (q 99999) (q () ) ))) (a))' '(123 056 789)'  
returns  
(99999)



---------------------------------------------------------------


// what puzzles look like //


:::::::::


puzzle_for_pk

code:  
def puzzle_for_pk(public_key):  
    aggsig = ConditionOpcode.AGG_SIG[0]  
    TEMPLATE = f"(c (c (q {aggsig}) (c (q 0x%s) (c (sha256 (wrap (a))) (q ())))) (a))"  
    return Program(binutils.assemble(TEMPLATE % hexbytes(public_key)))

"compiled":  
( ( (AGGSIG.OPCODE) ( (PUBKEY) (sha256 (wrap (a))) ) ) (a) )

e.g.  
(c (c (q 0x32) (c (q 0x91b985c21fde88ac3b0ce5037ade5ea2df295062919f085addf6dd3529eedb6b11af412fb7cd3af5183322872cc587cf) (c (sha256 (wrap (a))) (q ())))) (a))


---------------------------------------------------------------
