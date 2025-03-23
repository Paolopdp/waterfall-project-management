import { useTranslation } from "next-i18next";
import Link from "next/link";
import { useRouter } from "next/router";

const Navbar = () => {
  const { t } = useTranslation("common");
  const router = useRouter();

  const handleLogout = () => {
    // Clear the auth token cookie
    document.cookie =
      "auth_token=; path=/; expires=Thu, 01 Jan 1970 00:00:01 GMT";
    router.push("/login");
  };

  return (
    <nav className="bg-white shadow-sm">
      <div className="max-w-full mx-auto px-4">
        <div className="flex justify-between h-16">
          <div className="flex items-center">
            <Link href="/dashboard" className="flex-shrink-0 flex items-center">
              <img
                className="h-8 w-auto"
                src="/images/logo.jpg"
                alt="Waterfall Resource Manager"
              />
              <span className="ml-2 text-xl font-semibold">Waterfall RM</span>
            </Link>
          </div>

          <div className="flex items-center">
            <button
              type="button"
              className="ml-3 inline-flex items-center px-4 py-2 border border-transparent text-sm font-medium rounded-md text-gray-700 hover:bg-gray-100 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-indigo-500"
              onClick={handleLogout}
            >
              {t("common.logout")}
            </button>
          </div>
        </div>
      </div>
    </nav>
  );
};

export default Navbar;
