import { JSX } from "preact";
import { useState } from "preact/hooks";

export default function MenuBar() {
  const [activeTab, setActiveTab] = useState("search"); // 初期タブ

  const tabs = [
    { id: "search", nonActiveIcon: "/logo.svg", activeIcon: "/active-logo.svg", label: "検索" },
    { id: "collection", nonActiveIcon: "/logo.svg", activeIcon: "/active-logo.svg", label: "コレクション" },
    { id: "register", nonActiveIcon: "/logo.svg", activeIcon: "/active-logo.svg", label: "登録" },
    { id: "community", nonActiveIcon: "/logo.svg", activeIcon: "/active-logo.svg", label: "コミュニティ" },
    { id: "login", nonActiveIcon: "/logo.svg", activeIcon: "/active-logo.svg", label: "ログイン" },
  ];

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
        <div 
          key={tab.id} 
          style={{ 
            display: "flex", 
            flexDirection: "column", 
            alignItems: "center",
            cursor: "pointer" 
          }}
          onClick={() => setActiveTab(tab.id)}
        >
          <img 
            src={ activeTab === tab.id ? tab.activeIcon : tab.nonActiveIcon } 
            alt={tab.label} 
            style={{ width: "30px", height: "30px"}} 
          />
          <span style={{ fontSize: "12px", color: activeTab === tab.id ? "#6aac98" : "#fff" }}>{tab.label}</span>
        </div>
      ))}
    </div>
  );
};