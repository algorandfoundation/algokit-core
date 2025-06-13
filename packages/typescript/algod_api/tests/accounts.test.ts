import { describe, test, expect, beforeAll } from "vitest";
import { algorandFixture } from "@algorandfoundation/algokit-utils/testing";
import * as algodPackage from "@algorandfoundation/algokit-algod-api";

/**
 * Tests exercising Algod account-related endpoints.
 *
 * These mirror the Python template tests found under `custom_tests/test_accounts.py`.
 */
describe("Account API Tests", () => {
  const fixture = algorandFixture();
  let algodApi: algodPackage.AlgodApi;

  beforeAll(async () => {
    // Start a clean Algorand sandbox instance for each test run
    await fixture.newScope();

    // Configure the generated client to talk to the local sandbox
    const authConfig: algodPackage.AuthMethodsConfiguration = {
      api_key: "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa",
    };
    const serverConfig = new algodPackage.ServerConfiguration("http://localhost:4001", {});
    const configurationParameters = {
      httpApi: new algodPackage.IsomorphicFetchHttpLibrary(),
      baseServer: serverConfig,
      authMethods: authConfig,
      promiseMiddleware: [],
    } as any;
    const config = algodPackage.createConfiguration(configurationParameters);
    algodApi = new algodPackage.AlgodApi(config);
  });

  test("should get account information", async () => {
    const { testAccount } = fixture.context;
      const result = await algodApi.accountInformation(testAccount.addr.toString());

      expect(result).not.toBeNull();
      expect(result.address).toBe(testAccount.addr.toString());

      // Basic field validation
      expect(typeof (result as any).amount).toBe("number");
      expect(["Offline", "Online", "NotParticipating"]).toContain((result as any).status);
  });

  test("should get account information with exclude=all", async () => {
    const { testAccount } = fixture.context;
    const result = await algodApi.accountInformation(testAccount.addr.toString(), "json" as any, "all" as any);
    expect(result).not.toBeNull()
    expect(result.assets).toBeUndefined();
    expect(result.createdAssets).toBeUndefined();
    expect(result.appsLocalState).toBeUndefined();
    expect(result.createdApps).toBeUndefined();
  });
});
