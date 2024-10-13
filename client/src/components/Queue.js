import React, { useEffect, useState } from "react";
import { useParams } from "react-router-dom";

export default function Queue() {
  const { vhost, queue } = useParams();
  const [messages, setMessages] = useState([]);

  useEffect(() => {
    async function responses() {
      let response = await fetch(`/queues/${vhost}/${queue}?count=1`);
      setMessages(await response.json());
    }
    responses();
  }, [vhost, queue]);

  return (
    <>
      <h1>Vhost: {vhost}</h1>
      <h1>Queue: {queue}</h1>
    </>
  );
}
