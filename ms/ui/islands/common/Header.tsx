import { useEffect, useState } from "preact/hooks";

export default function Header() {
  const [redirectPath, setRedirectPath] = useState<string>("");

  useEffect(() => {
    const path = window.location.pathname;
    const deviceType = path.split("/")[1];
    setRedirectPath(`/${deviceType}`);
  }, []);

  return (
    <header
      style={{
        display: "flex",
        flexDirection: "column",
        alignItems: "center",
        justifyContent: "center",
        textAlign: "center",
        fontFamily: "hiragino kaku gothic pro, sans-serif",
      }}
    >
      <a href={redirectPath} style={{ textDecoration: "none", color: "black" }}>
        <p style={{ fontSize: "32px" }}>COSAN</p>
      </a>
    </header>
  );
}