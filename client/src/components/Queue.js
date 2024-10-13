import React from "react";
import { useParams } from "react-router-dom";

export default function Queue() {
  const {vhost, queue} = useParams();

  return (
    <>
      <h1>Vhost: {vhost}</h1>
      <h1>Queue: {queue}</h1>
    </>
  )
}
