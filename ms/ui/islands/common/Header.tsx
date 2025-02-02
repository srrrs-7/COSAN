import { useEffect, useState } from "preact/hooks";

export default function Header() {
  const [redirectPath, setRedirectPath] = useState<string>("");

  useEffect(() => {
    const path = window.location.pathname;
    const deviceType = path.split("/")[1];
    setRedirectPath(`/${deviceType}/mypage`);
  }, []);

  return (
    <header style={{ backgroundColor: "#4a5568", color: "white", padding: "10px", textAlign: "center" }}>
      <a href={redirectPath} style={{ color: "white", textDecoration: "none", fontSize: "20px", fontWeight: "bold" }}>
        COSAN
      </a>
    </header>
  );
}