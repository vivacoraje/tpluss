select code from DI_Distribution where DateDiff(dd, voucherdate, getdate())=-1;

select * from DI_Distribution where DateDiff(dd, voucherdate, getdate())=-1 AND code='DT.2022.03.18.003';

select code, sourcevouchercode, isnomodify, exchangerate, memo, maker, auditor, reviser, iscarriedforwardout, iscarriedforwardin, ismodifiedcode, sequencenumber, ts, pubuserdefnvc1, PrintCount, ID, idbusitype, idcurrency, idmarketingorgan, idcustomer, idsettleCustomer, idoutwarehouse, sourcevoucherid, completestatus, voucherstate, makerid, idsourcevouchertype, madedate, createdtime, updated from DI_Distribution where DateDiff(dd, voucherdate, getdate())=-1 AND code='DT.2022.03.18.003';

select top 10 * from DI_Distribution_b where idDistributionDTO=192723;

            SELECT 
               sb.quantity, sb.quantity2, sb.idinventory, sb.idbaseunit, sb.idsubunit, sb.idunit, sb.idunit2, imu.idunit1, imu.unit1Name
            FROM 
                (SA_SaleDelivery_b AS sb 
                JOIN AA_Inventory AS iv ON sb.idinventory=iv.id) join AA_Inventory_MultiUnit as imu ON iv.id=imu.IDInventory
            WHERE idSaleDeliveryDTO= 637460

/*UPDATE SA_SaleDelivery SET PrintCount = 1 WHERE code = 'SA.2022.03.18.0054';

CREATE TABLE 'deliverer' (
	'id' INTEGER PRIMARY KEY AUTOINCREMENT,
	'name' CHAR(20) NOT NULL
);


CREATE TABLE 'payment' (
	'id' INTEGER PRIMARY KEY AUTOINCREMENT,
	'name' CHAR(10)
);


CREATE TABLE 'delivery' (
	'id' INTEGER PRIMARY KEY AUTOINCREMENT,
	'code' CHAR(20) NOT NULL UNIQUE,
	'customer' CHAR(128) NOT NULL UNIQUE,
	'amount' DECIMAL(10, 3) NOT NULL,
	'payment_amount' DECIMAL(10, 3) NOT NULL,
	'memo' CHAR(128),
	'quantity' DECIMAL(5, 3) NOT NULL,
	'created_at' DATETIME DEFAULT (datetime('now', 'localtime')),
	'location_at' CHAR(128),
	'deliverer_id' INTEGER,
	'payment_id' INTEGER,
	FOREIGN KEY ('deliverer_id') REFERENCES 'deliverer' ('id'),
	FOREIGN KEY ('payment_id') REFERENCES 'payment' ('id')
);

--DROP TABLE 'deliverer';
--DROP TABLE 'payment';
--DROP TABLE 'delivery';
--VACUUM;

INSERT INTO 'payment' (name) VALUES
('??????'),
('??????'),
('?????????'),
('??????'),
('??????'),
('????????????'),
('??????'),
('????????????'),
('????????????'),
('??????');

--'??????', '??????', '?????????', '??????', '??????', '????????????', '??????', '????????????', '????????????', '??????'

INSERT INTO 'deliverer' (name) VALUES
('whg'),
('lxw'),
('ybk'),
('zl'),
('zxp'),
('mwd');

INSERT INTO user (name, role_id)
SELECT "LUCY", id 
FROM role WHERE id=1;

INSERT INTO "user" (name, role_id)
SELECT "BOSS", id 
FROM role WHERE name = 'super-user';
