import { JSX } from "preact";
import { useState, useEffect } from "preact/hooks";

export default function MenuBar() {
  const [activeTab, setActiveTab] = useState("search");

  const tabs = [
    { id: "search", nonActiveIcon: "/logo.svg", activeIcon: "/active-logo.svg", label: "検索", redirectPath: "/www/mypage/search" },
    { id: "mypage", nonActiveIcon: "/logo.svg", activeIcon: "/active-logo.svg", label: "登録", redirectPath: "/www/mypage" },
    { id: "www", nonActiveIcon: "/logo.svg", activeIcon: "/active-logo.svg", label: "ログイン/マイページ", redirectPath: "/www" },
  ];

  useEffect(() => {
    const path = window.location.pathname;
    const paths = path.split("/");
    const tabId = paths[paths.length - 1];
    setActiveTab(tabId);
  }, []);

  return (
    <div style={{ 
      position: "fixed", 
      bottom: 0, 
      width: "100%", 
      backgroundColor: "#bbb",
      padding: "10px",
      display: "flex",
      justifyContent: "space-around" 
    }}>
      {tabs.map(tab => (
        <a href={tab.redirectPath} style={{ textDecoration: "none", color: "black" }}>
          <div 
            key={tab.id} 
            style={{ 
              display: "flex", 
              flexDirection: "column", 
              alignItems: "center",
              cursor: "pointer" 
            }}
          >
            <img 
              src={ activeTab === tab.id ? tab.activeIcon : tab.nonActiveIcon } 
              alt={tab.label} 
              style={{ width: "30px", height: "30px"}} 
            />
            <span style={{ fontSize: "12px", color: activeTab === tab.id ? "#6aac98" : "#fff" }}>{tab.label}</span>
          </div>
        </a>
      ))}
    </div>
  );
};