import "../../node_modules/@oresoftware/ores-contracts/typespec/ores.tsp";
import "./scalars.tsp";
using Ores;
using HhausScalars;

namespace Hhaus.Public;

enum ClientPlatform {
  Browser: "browser",
  Desktop: "desktop",
  Ios: "ios",
  Android: "android",
}

enum AccountKind {
  Individual: "individual",
  Organization: "organization",
}

enum LocationAvailability {
  AcceptingReservations: "accepting-reservations",
  Waitlist: "waitlist",
  Closed: "closed",
}

@doc("Public metadata for one client installation; never an authenticated session or credential.")
@Ores.table("public_client_installations")
model ClientInfo {
  @key @maxLength(128) installId: string;
  platform: ClientPlatform;
  @maxLength(64) appVersion: string;
  @maxLength(16) locale?: string;
}

@doc("A client-generated replay-protection key, not proof that a mutation completed.")
@Ores.table("public_mutation_keys")
model IdempotencyKey {
  @key @maxLength(128) key: string;
  mintedAtMs: int64;
}

@doc("Public, non-sensitive location catalog projection.")
@Ores.table("public_location_catalog")
model PublicLocation {
  @key @maxLength(64) slug: string;
  @maxLength(120) displayName: string;
  @maxLength(80) city: string;
  @maxLength(2) countryCode: string;
  @maxLength(64) timezone: string;
  availability: LocationAvailability;
}

@doc("Minimal public account context for B2C individuals and B2B organizations.")
@Ores.table("public_account_contexts")
model PublicAccountContext {
  @key id: uuid;
  kind: AccountKind;
  @maxLength(120) displayName: string;
}

