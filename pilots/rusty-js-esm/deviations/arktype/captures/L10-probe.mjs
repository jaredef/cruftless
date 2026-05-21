// L1: bypass arktype's user-facing API; import @ark/schema directly.
import * as S from '@ark/schema';
console.log(JSON.stringify({status:'OK', keyCount: Object.keys(S).length}));
