import "./App.css";
import React, { useState } from "react";

const wasmPromise = import("wasm");

async function getAccountData(addressStr) {
  const { get_account_data } = await wasmPromise;
  return get_account_data(addressStr);
}

function App() {
  const [addressInput, setAddressInput] = useState("");
  const [account, setAccount] = useState(null);

  let accountView;
  if (account != null) {
    accountView = <AccountViewData viewData={account} />;
  } else {
    accountView = <div />;
  }

  return (
    <div className="App">
      <input
        type="text"
        size="64"
        name="adress"
        onChange={(event) => {
          setAddressInput(event.target.value);
        }}
      />
      <input
        type="submit"
        value="Submit"
        onClick={async (event) => {
          setAccount(await getAccountData(addressInput));
        }}
      />
      {accountView}
    </div>
  );
}

function AccountViewData(props) {
  let viewData = props.viewData;
  return (
    <div>
      {/* <div>Address: {viewData.address}</div> */}
      <div>Status: {viewData.status}</div>
      <div>Holdings: {viewData.holdings}</div>
      <div>Rewards: {viewData.rewards}</div>
      <div>Pending rewards: {viewData.pending_rewards}</div>
    </div>
  );
}

export default App;
