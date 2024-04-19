// import "module-name";
import * as name from "module-name";

// import defaultExport from "module-name";
import { export1 } from "module-name";
// import { export1 as alias1 } from "module-name";
// import { default as alias } from "module-name";

import { export1, export2 } from "module-name";
// import { export1, export2 as alias2, } from "module-name";
// import defaultExport, { export1, } from "module-name";
// import defaultExport, * as name from "module-name";

// import { Person } from './person.js', { Animal } from './animal.js';

// const moduleName = 'person';
// import { Person } from `./${moduleName}.js`;


function addNumbers(a, b) {
    return a + b;
  }
  
const result = addNumbers(5, 3);
console.log(result); 
