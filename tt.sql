select code from DI_Distribution where DateDiff(dd, voucherdate, getdate())=-1;

select * from DI_Distribution where DateDiff(dd, voucherdate, getdate())=-1 AND code='DT.2022.03.18.003';

select code, sourcevouchercode, isnomodify, exchangerate, memo, maker, auditor, reviser, iscarriedforwardout, iscarriedforwardin, ismodifiedcode, sequencenumber, ts, pubuserdefnvc1, PrintCount, ID, idbusitype, idcurrency, idmarketingorgan, idcustomer, idsettleCustomer, idoutwarehouse, sourcevoucherid, completestatus, voucherstate, makerid, idsourcevouchertype, madedate, createdtime, updated from DI_Distribution where DateDiff(dd, voucherdate, getdate())=-1 AND code='DT.2022.03.18.003';

select top 10 * from DI_Distribution_b where idDistributionDTO=192723;

/*UPDATE SA_SaleDelivery SET PrintCount = 1 WHERE code = 'SA.2022.03.18.0054';

