// This is a function for later use if deemed necessary
// problem is that impossible to tell if tx is legit for frontrun without checking blocktimes
// but this is hard (probably not a good idea) because confirmation times can be long



	event Repaid(
		address indexed creditor,
		uint256 value );

	event TooMuchDebt();

	event InsufficientBalanceAfterRepay();


		// (completely (almost)) eliminating double withdrawal vulnerability
	mapping(address => bool) public owes;
	mapping(address => mapping(address => uint256)) private _allowanceTotalIncrease;
	mapping(address => mapping(address => uint256)) private _transferFromTotal;
	struct Credits {
		address creditor;
		uint256 owed; }
	Credits[] private _credits;
	mapping(address => _credits) private _creditorsOwed;



		   // from the moment they submit valid tx, not when added to block
		  // this contract honors the intent of an approver
		 // if you aren't but got unlucky, sorry
		// pay everybody back if you are a double withdrawer
	function paybackDoubleWithdrawalDebts(
		address debtor,
		uint256 amount
	) public returns (bool) {

			// pay back your dirty debt. You can't do a thing until you pay it off.
		if (owes[debtor]) {

			// settle individual double withdrawal debts
			// note that this is only really happening if somebody has cheated
			// so I am not worried about expensive gas iterating and popping so much
			for (uint16 i = 0, i < _creditorsOwed[debtor].length, i++) {
				if (_creditorsOwed[debtor][i].owed > _balances[debtor]) {

					_creditorsOwed[debtor][i].owed -= _balances[debtor]; 
					_transfer(
						owner,
						creditorsOwed[debtor][i].creditor,
						_balances[debtor]
					);
					emit Repaid(
						creditorsOwed[debtor][i].creditor,
						_balances[debtor]
					);
					emit TooMuchDebt();
					for (uint16 index = i, index < _creditorsOwed[owner].length - i, index++) {
						_creditorsOwed[debtor][index - i] = _creditorsOwed[debtor][index];
					}
					for (uint16 index = 0, index < i, index++) {
						_creditorsOwed[debtor].pop();
					}

					return true;
				}
				transfer(
					owner,
					creditorsOwed[debtor][i].creditor,
					creditorsOwed[debtor][i].owes,
				);
				emit Repaid(
					creditorsOwed[debtor][i].creditor,
					creditorsOwed[debtor][i].owes,
				);
				
			}	
			owes[debtor] = false;
			for (uint16 i = 0, i < _creditorsOwed[debtor].length, index++) {
				_creditorsOwed[debtor].pop();
			}
			if (ammount > _balances[debtor]) {
				emit InsufficientBalanceAfterRepay();
				return true;
			}
		}	

