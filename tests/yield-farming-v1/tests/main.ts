/**
 * Test for yield farming
 */

import { testCreateFarm } from "./test_create_farm";

async function main() {
    console.log("-------------test started--------------");

    await testCreateFarm();

}



main().then(
() => console.error("-------------test   ended--------------"),
err => {
    console.error(err);
},
);