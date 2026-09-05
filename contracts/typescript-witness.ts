import type {
  ClientInfo,
  PublicAccountContext,
  PublicLocation,
} from "@hacker-house-medellin/hhm-pub-lib-core";

const client: ClientInfo = {
  installId: "install-1",
  platform: "browser",
  appVersion: "1.0.0",
};

const location: PublicLocation = {
  slug: "medellin",
  displayName: "H/HAUS Medellin",
  city: "Medellin",
  countryCode: "CO",
  timezone: "America/Bogota",
  availability: "accepting-reservations",
};

const account: PublicAccountContext = {
  id: "9e1c98d5-80d8-40cc-97b1-9a09286cfbe7",
  kind: "organization",
  displayName: "Example organization",
};

void [client, location, account];

