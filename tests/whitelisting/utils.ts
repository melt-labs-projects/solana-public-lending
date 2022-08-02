import * as pda from './pda';


export const isInitialised = async (program) => {
    const [admin, ] = await pda.findAdmin(program);
    try {
        const adminData = await program.account.adminAccount.fetch(admin);
    } catch (e) {
        return false;
    }
    return true;
}
