import { YieldFarm } from "./utils/farm";
import moment from "moment";
import { airdropAndMakeTokens, B2BCRPAmmPubkey, B2BCRPLPPubkey, B2BMINTPubkey, connection, CREATOR_WALLET_ACCOUNT, delay, FarmProgramPubkey } from "./setup_for_test";

export async function testCreateFarm(){
    await airdropAndMakeTokens();
    await delay(5000);

    const _connection = connection;
    const _walletAccount = CREATOR_WALLET_ACCOUNT;
    let current = moment().unix();
    let future10days = current + 10 * 24 * 3600;
    let future50days = current + 50 * 24 * 3600;
    let past10days = current - 10 * 24 * 3600;
    let past50days = current - 50 * 24 * 3600;

    let rewardMintPubkey = B2BMINTPubkey;
    let lpMintPubkey = B2BCRPLPPubkey;
    let ammPubkey = B2BCRPAmmPubkey;

    console.log("creating test farms ...")

    let notStartedFarm:any;
    let pastStartedFarm:any;
    let endEarlierFarm:any;
    let currentStartedFarm:any;
    let pastCurrentFarm:any;
    let pastPastFarm:any;
    
    try{
        // not started farm
        notStartedFarm = await YieldFarm.createFarmWithParams(
            _connection,
            _walletAccount,
            rewardMintPubkey,
            lpMintPubkey,
            ammPubkey,
            future10days,
            future50days
        );
    }
    catch{

    }
    try{
        // past started farm
        pastStartedFarm = await YieldFarm.createFarmWithParams(
            _connection,
            _walletAccount,
            rewardMintPubkey,
            lpMintPubkey,
            ammPubkey,
            past10days,
            future50days
        );
    }
    catch{

    }
    try{
        // past started farm
        endEarlierFarm = await YieldFarm.createFarmWithParams(
            _connection,
            _walletAccount,
            rewardMintPubkey,
            lpMintPubkey,
            ammPubkey,
            future50days,
            future10days
        );

    }
    catch{

    }
    try{
        // current started farm
        currentStartedFarm = await YieldFarm.createFarmWithParams(
            _connection,
            _walletAccount,
            rewardMintPubkey,
            lpMintPubkey,
            ammPubkey,
            current,
            future50days
        );
    }
    catch{

    }
    try{
        // past current farm
        pastCurrentFarm = await YieldFarm.createFarmWithParams(
            _connection,
            _walletAccount,
            rewardMintPubkey,
            lpMintPubkey,
            ammPubkey,
            past50days,
            current
        );
    }
    catch{

    }
    try{
        // past past farm
        pastPastFarm = await YieldFarm.createFarmWithParams(
            _connection,
            _walletAccount,
            rewardMintPubkey,
            lpMintPubkey,
            ammPubkey,
            past50days,
            past10days
        );
    }
    catch{

    }
    await delay(20000);

    console.log("loading created farms ...")
    let passed = 0;
    let failed = 0;
    try{
        let fetchedNotStartedFarm = await YieldFarm.loadFarm(
            connection,
            notStartedFarm.farmId,
            FarmProgramPubkey
        )
        console.log("loaded fetchedNotStartedFarm"," --- passed")
        passed++;
    }
    catch{
        console.log("failed fetchedNotStartedFarm"," --- failed")
        failed++;
    }

    try{
        let fetchedPastStartedFarm = await YieldFarm.loadFarm(
            connection,
            pastStartedFarm.farmId,
            FarmProgramPubkey
        )
        console.log("loaded fetchedPastStartedFarm"," --- passed")
        passed++;
    }
    catch{
        console.log("failed fetchedPastStartedFarm"," --- failed")
        failed++;
    }

    try{
        let fetchedEndEarlierFarm = await YieldFarm.loadFarm(
            connection,
            endEarlierFarm.farmId,
            FarmProgramPubkey
        )
        console.log("loaded fetchedEndEarlierFarm"," --- failed")
        failed++;
    }
    catch{
        console.log("failed fetchedEndEarlierFarm"," --- passed")
        passed++;
    }

    try{
        let fetchedCurrentStartedFarm = await YieldFarm.loadFarm(
            connection,
            currentStartedFarm.farmId,
            FarmProgramPubkey
        )
        console.log("loaded fetchedCurrentStartedFarm"," --- passed")
        passed++;
    }
    catch{
        console.log("failed fetchedCurrentStartedFarm"," --- failed")
        failed++;
    }

    try{
        let fetchedPastCurrentFarm = await YieldFarm.loadFarm(
            connection,
            pastCurrentFarm.farmId,
            FarmProgramPubkey
        )
        console.log("loaded fetchedPastCurrentFarm"," --- passed")
        passed++;
    }
    catch{
        console.log("failed fetchedPastCurrentFarm"," --- failed")
        failed++;
    }
    try{
        let fetchedPastPastFarm = await YieldFarm.loadFarm(
            connection,
            pastPastFarm.farmId,
            FarmProgramPubkey
        )
        console.log("loaded fetchedPastPastFarm"," --- passed")
        passed++;
    }
    catch{
        console.log("failed fetchedPastPastFarm"," --- failed")
        failed++;
    }


    console.log("passed = ",passed," failed=",failed);
}