use near_sdk::{env, near, require, AccountId, NearToken, PanicOnDefault, Promise};
use std::collections::HashMap;
use uuid::Uuid;

#[near(serializers = [json, borsh])]
#[derive(Clone)]
pub struct Employer
{
  name: String,
  account: AccountId,
  employees: HashMap<Uuid,Employee>
}

#[near(serializers = [json, borsh])]
#[derive(Clone)]
pub struct Employee
{
  account: AccountId,
  salary: NearToken,
  payed: bool
}

#[near(contract_state)]
#[derive(PanicOnDefault)]
pub struct Contract
{
  employers: HashMap<Uuid,Employer>
}

#[near]
impl Contract
{
  #[init]
  #[private]
  pub fn init() -> Self
  {
    Self
    {
      employers: HashMap::new()
    }
  }

  pub fn add_employer(&mut self, name:String, account:AccountId)
  {
    let _employer = Employer
    {
      name,
      account,
      employees: HashMap::new()
    };
    self.employers.insert(Uuid::new_v4(), _employer);
  }

  pub fn add_employee(&mut self, employer_id:String, account: AccountId, salary:NearToken)
  {
    //TODO: Check if the employer_id is valid!
    require!(
      self.employers.contains_key(&Uuid::try_parse(&employer_id).unwrap())
      ,"Employer must exist before employee."
    );

    let _employee = Employee {
      account,
      salary,
      payed:false
    };

    self.employers.get_mut(&Uuid::try_parse(&employer_id).unwrap()).unwrap().employees.insert(
      Uuid::new_v4(),
      _employee
    );
  }

  pub fn pay_employees(&mut self, employer_id:String)
  {
    for (_id, employee) in self.employers.get_mut(&Uuid::try_parse(&employer_id).unwrap()).unwrap().employees.clone()
    {
      self.pay_employee(employee);
    }
  }

  fn pay_employee(&mut self, mut employee:Employee) -> Promise {
    require!(
      employee.payed == false,
      "Employee can only be payed once per pay period"
    );

    employee.payed = true;
    Promise::new(employee.account).transfer(employee.salary)
  }

  #[payable]
  pub fn fund_payroll(&mut self) -> Promise {
    let budget = env::attached_deposit();
    let account = env::predecessor_account_id();
    Promise::new(account).transfer(budget)
  }
}

/*
 * The rest of this file holds the inline tests for the code above
 * Learn more about Rust tests: https://doc.rust-lang.org/book/ch11-01-writing-tests.html
 */
/* #[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_default_greeting() {
        let contract = Contract::default();
        // this test did not call set_greeting so should return the default "Hello" greeting
        assert_eq!(contract.get_greeting(), "Hello");
    }

    #[test]
    fn set_then_get_greeting() {
        let mut contract = Contract::default();
        contract.set_greeting("howdy".to_string());
        assert_eq!(contract.get_greeting(), "howdy");
    }
} */
