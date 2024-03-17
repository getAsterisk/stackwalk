from mymodule import obj1, obj2, obj3

from mymodule import *

import mymodule
result = mymodule.func1(arg1, arg2)

from mymodule import my_object as obj

from mymodule import obj1 as o1, obj2 as o2, func1 as f1

import mymodule as mm
result = mm.func1(arg1, arg2)

from mypackage.mymodule import my_object

from ..parent_package import my_object

from mymodule import (
   obj1,
   obj2,
   obj3,
)

from mymodule import (
   obj1 as o1,
   obj2 as o2,
   func1 as f1,
)
