import { ReactNode } from "react";

export default function PageLayout({ children }: { children: ReactNode }) {
    return (
      <div>
        <h2>My Dashboard</h2>
        {children}
      </div>
    );
  }