import JsonView from "@uiw/react-json-view";
import { useEffect, useState } from "react";

export default function App() {
  const [vhosts, setVhosts] = useState([]);

  useEffect(() => {
    async function responses() {
      let response = await fetch("/vhosts");
      setVhosts(await response.json());
    }
    responses();
  }, []);

  return (
    <>
      <JsonView value={vhosts} />
    </>
  );
}
