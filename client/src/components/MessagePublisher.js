import { useParams } from "react-router-dom";
import Vhosts from "./Vhosts";
import { useState } from "react";

/// Allows users to select a vhost, obtain a list of exchanges,
///  select an exchange, and publish a message to that exchange.
export default function MessagePublisher({ vhosts }) {
  const [vhost] = useParams();
  const [selectedVhost, setSelectedVhost] = useState("");

  /**
   * Loads the exchanges based on the selected vhost, and sets the state.
   * @param {*} event
   */
  const handleVhostChange = async (event) => {
    let response = await fetch(`/exchanges/${event.target.value}`);
    let exchanges = await response.json();
    setSelectedVhost(exchanges);
  };

  return (
    <>
      <Vhosts
        vhosts={vhost}
        selectedVhost={selectedVhost}
        handleChange={handleVhostChange}
      />
    </>
  );
}
