import { waffle } from '@nomiclabs/buidler';
import chai from 'chai';
import { deployContract, solidity } from 'ethereum-waffle';
import { utils } from 'ethers';

import RecurrenceArtifact from '../artifacts/Recurrence.json';
import StarkDigestTestingArtifact from '../artifacts/StarkDigestTesting.json';
import { Recurrence } from '../typechain/Recurrence';
import { StarkDigestTesting } from '../typechain/StarkDigestTesting';

import recurrence_proofs from './recurrence_proofs.json';

const INITIAL_GAS = 100000000;

chai.use(solidity);

describe('Recurrence testing', function (this: any): void {
    // Disables the timeouts
    this.timeout(0);
    let constraint_contract: Recurrence;
    let verifier_contract: StarkDigestTesting;

    const provider = waffle.provider;
    const [wallet] = provider.getWallets();

    before(async () => {
        constraint_contract = (await deployContract(wallet, RecurrenceArtifact)) as Recurrence;
        verifier_contract = (await deployContract(wallet, StarkDigestTestingArtifact)) as StarkDigestTesting;
    });

    // Note - This checks the proof of work, but not the whole proof yet
    it.only('It should validate a correct proof', async () => {
        for (let i = 19; i < recurrence_proofs.length; i++) {
            // We ts-ignore because it's connivent to abi encode here not in rust
            // @ts-ignore
            recurrence_proofs[i].public_inputs = utils.defaultAbiCoder.encode(
                ['uint256', 'uint64'],
                [recurrence_proofs[i].public_inputs.value, recurrence_proofs[i].public_inputs.index],
            );
            // NOTE - Typescript has a very very hard time with the ethers js internal array types in struct encoding
            // in this case it's best for the code to ignore it because this is how ethers js understands these types.
            // @ts-ignore
            const receipt = await (
                // @ts-ignore
                await verifier_contract.verify_proof(recurrence_proofs[i], constraint_contract.address, { gasLimit: INITIAL_GAS })
            ).wait();
            console.log(`ENTER transaction ${INITIAL_GAS} 0`);
            var lastAlloc = 0;
            for (const event of receipt.events) {
                if (event.event != 'LogTrace') {
                    continue;
                }
                const direction = event.args.enter ? 'ENTER' : 'LEAVE';
                const name = utils.parseBytes32String(event.args.name);
                console.log(`${direction} ${name} ${event.args.gasLeft} ${event.args.allocated}`);
                lastAlloc = event.args.allocated;
            }
            console.log(`LEAVE transaction ${INITIAL_GAS - receipt.gasUsed?.toNumber()} ${lastAlloc}`);
            // TODO - Use better logging
            /* tslint:disable:no-console*/
            console.log('Proof verification gas used : ', receipt.gasUsed?.toNumber());
        }
    });
});
