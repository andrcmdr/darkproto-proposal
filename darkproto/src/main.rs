// use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use threshold_crypto::{
    Ciphertext, DecryptionShare, PublicKey, PublicKeySet, PublicKeyShare, SecretKeySet, SecretKeyShare,
};

/// For logging initialization
use tracing:: {info, debug, error};
use cli::{init_logging, Error};

/// The asset (content) or content password for encryption
use cat::GEORGE as G;

pub mod cli;
pub mod cat;

/// The "trusted key dealer" (trusted operator/platform) is responsible for initialization of swarm data (node_id, SK and PK fragments)
/// and key generation for each encrypted asset with assigned content_id.
/// content_id, node_IDs, pk_set, ciphertext are stored on a trusted operator side, data can be distributed, sharded and stored
/// as mmutable block-chain data structure in DB.

/// Encrypted files (content/data) with content_id, master public key of swarm (for cryptosystem configuration and encryption),
/// content producer public key and signature, trusted operator service node public key and signature, are stored in a distributed file storage.

/// For the sake of simplicity we use actor model here as an emulation of swarm network and data exchange between trusted operator and carrier nodes.

/// Swarm is a pool of carrier nodes for distributed storage of private and public key fragments (shares).
#[derive(Clone, Debug)]
struct Swarm {
    actors: Vec<Actor>,
    pk_set: PublicKeySet,
}

impl Swarm {
    /// Creates a new swarm of actors as key fragments carriers
    /// Init swarm:
    /// `actors_num` - set maximal amount of generated key fragments and thus number of carrier nodes in a swarm;
    /// `threshold` - set threshold, minimal amount of key fragments and nodes to decrypt message ciphertext content,
    /// the number of nodes must exceed this `threshold`, i.e. minimal abount of nodes for collaborative/collective decryption is `threshold+1`;
    fn new(actors_num: usize, threshold: usize) -> Self {
        let mut rng = rand::thread_rng();

        // Generate SK_set and PK_set;
        let sk_set = SecretKeySet::random(threshold, &mut rng);
        debug!(target: "darkproto", "SK set has been generated: {:?}\n", sk_set);
        let pk_set = sk_set.public_keys();
        debug!(target: "darkproto", "PK set has been generated from SK set: {:?}\n", pk_set);

        // Create actors which are carrying key fragments
        let actors = (0..actors_num)
            .map(|id| {
                // Generate actors/nodes IDs, SK & PK fragments/shares;
                let sk_share = sk_set.secret_key_share(id);
                debug!(target: "darkproto", "SK share {:?} for actor id {} has been generated from SK set\n", sk_share, id);
                let pk_share = pk_set.public_key_share(id);
                debug!(target: "darkproto", "PK share {:?} for actor id {} has been generated from PK set\n", pk_share, id);
                debug!(target: "darkproto", "Actor with id: {}, SK share: {:?}, PK share: {:?} has been created\n", id, sk_share, pk_share);
                Actor::new(id, sk_share, pk_share)
            })
            .collect();

        debug!(target: "darkproto", "Swarm with actors: {:?} and PK set: {:?} has been created\n", actors, pk_set);
        Swarm { actors, pk_set }
    }

    /// Get the swarm Master Public Key
    fn publish_public_key(&self) -> PublicKey {
        self.pk_set.public_key()
    }

    /// Get particular actor/node data from a swarm
    fn get_actor(&mut self, id: usize) -> &mut Actor {
        self.actors
            .get_mut(id)
            .expect("No `Actor` exists with that ID")
    }

    // Starts a new decryption meeting for the swarm. Each time the set of actors receive an encrypted
    // message, at least `threshold+1` of them (i.e. 1 more than the threshold) must work together to decrypt
    // the ciphertext.
    fn start_decryption_meeting(&self) -> DecryptionMeeting {
        DecryptionMeeting {
            pk_set: self.pk_set.clone(),
            ciphertext: None,
            dec_shares: BTreeMap::new(),
        }
    }
}

/// A member of the nodes swarm - carrier node/actor.
/// Carrier node stores:
/// Node ID,
/// SK fragment,
/// PK fragment,
/// msg_box (mainly for ciphertext receive),
/// for each encrypted asset with assigned content ID
#[derive(Clone, Debug)]
struct Actor {
    id: usize,
    sk_share: SecretKeyShare,
    pk_share: PublicKeyShare,
    msg_inbox: Option<Ciphertext>,
}

impl Actor {
    /// Create new actor
    fn new(id: usize, sk_share: SecretKeyShare, pk_share: PublicKeyShare) -> Self {
        Actor {
            id,
            sk_share,
            pk_share,
            msg_inbox: None,
        }
    }
}

// Sends an encrypted message (ciphertext) to an `Actor`.
fn send_msg(actor: &mut Actor, enc_msg: Ciphertext) {
    actor.msg_inbox = Some(enc_msg);
}

/// Decryption meeting of actors/nodes in a swarm. At this meeting, actors collaborate together to decrypt the ciphertext.
#[derive(Clone, Debug)]
struct DecryptionMeeting {
    pk_set: PublicKeySet,
    ciphertext: Option<Ciphertext>,
    dec_shares: BTreeMap<usize, DecryptionShare>,
}

impl DecryptionMeeting {
    /// An actor contributes their decryption share to the decryption process.
    fn accept_decryption_share(&mut self, actor: &mut Actor) {
        let ciphertext = actor.msg_inbox.take().unwrap();

        // Check that the actor's ciphertext is the same ciphertext decrypted at the meeting.
        // The first actor to arrive at the decryption meeting sets the meeting's ciphertext.
        if let Some(ref meeting_ciphertext) = self.ciphertext {
            if ciphertext != *meeting_ciphertext {
                return;
            }
        } else {
            self.ciphertext = Some(ciphertext.clone());
        }

        // Request from actor/node SK share and generate decryption share/fragment from it,
        // verify/validate decryption share using PK share,
        // collect decryption shares into decryption meeting data structure;
        let dec_share = actor.sk_share.decrypt_share(&ciphertext).unwrap();
        debug!(target: "darkproto", "Decrypt SK share {:?} and generate decryption share {:?} using corresponbding valid ciphertext.\n", &actor.sk_share, &dec_share);
        let dec_share_is_valid = actor
            .pk_share
            .verify_decryption_share(&dec_share, &ciphertext);
        debug!(target: "darkproto", "Verify decryption share {:?} using corresponbding PK share {:?} and valid ciphertext.\n", &actor.pk_share, &dec_share);
        assert!(dec_share_is_valid);
        debug!(target: "darkproto", "Decryption share is valid. Accept decryption share {:?} for actor id {} in a decryption meeting.\n", &dec_share, &actor.id);
        self.dec_shares.insert(actor.id, dec_share);
    }

    // Tries to decrypt the shared ciphertext using the decryption shares.
    fn decrypt_message(&self) -> Result<Vec<u8>, Error> {
        let ciphertext = self.ciphertext.clone().unwrap();
        // Use PK_set and decryption shares/fragments to decrypt ciphertext, i.e. retrieve Node_SS = CP_SS ephemeral mutual symmetric key;
        debug!(target: "darkproto", "Use PK set {:?} and decryption shares {:?} to decrypt corresponding ciphertext.\n", &self.pk_set, &self.dec_shares);
        self.pk_set
            .decrypt(&self.dec_shares, &ciphertext)
            .map_err(|e| {
                error!(target: "darkproto", "Error: {:?}\n", e);
                format!("Error: {:?}", e).into()
            })
    }
}

fn main() {

    // Initialize logging
    init_logging();

    // Waiting for `Enter` key press
    // let mut buffer = String::new();
    // let stdin = std::io::stdin();
    // info!(target: "darkproto", "Press `Enter` to continue:");
    // stdin.read_line(&mut buffer).expect("Failed to read input");

    // Waiting 10 seconds in a main thread
    info!(target: "darkproto", "Waiting 10 seconds in a main thread...");
    std::thread::sleep(core::time::Duration::from_millis(10000));

    // Creates a Swarm with up to 3 actors and threshold equal to 2 actors.
    // Any message encrypted with the swarm master public-key (ciphertext, encrypted content secret)
    // will require 2 or more actors working together to decrypt (i.e. the decryption threshold is 1).
    // Once the swarm has created its private/public keys set and master public key,
    // it generates a secret-key share and public-key share fragments to each of its actors.
    // The swarm then publishes its master public key to a publicly acessible distributed file storage.
    let actors_num = 3_usize;
    let threshold = 1_usize;
    let mut swarm = Swarm::new(actors_num, threshold);
    debug!(target: "darkproto", "Swarm of up to {} nodes/actors and threshold of {} nodes/actors successfully created\n", actors_num, threshold);
    debug!(target: "darkproto", "Actors has been created in a swarm: {:?}\n", swarm);

    // Retrieve master public key for swarm;
    // Send Master Public Key and encrypted content file to distributed storage;
    let pk = swarm.publish_public_key();
    debug!(target: "darkproto", "Swarm public key has been published: {:?}\n", pk.0);

    // encrypt message, containing secret key for access to encrypted asset content file, using swarm's master public-key,
    // and send the ciphertext to each of the carrier actors/nodes in a swarm

    let msg = G.as_bytes();
    info!(target: "darkproto", "Plain text message: {}\n", String::from_utf8_lossy(msg));

    // Encrypt content file with service node shared secret (Node_SS), equal to content producer shared secret (CP_SS),
    // and which is an ephemeral mutual symmetric key, derived during ECDH key exchange session;
    // Encrypt Node_SS = CP_SS with Master PK, generate ciphertext for nodes swarm;
    let ciphertext = pk.encrypt(msg);
    debug!(target: "darkproto", "Message, which contains content secret key, has been encrypted with master public key of a swarm\n");

    info!(target: "darkproto", "Ciphertext: {:?},{:?},{}\n", &ciphertext.0, &ciphertext.2, String::from_utf8_lossy(&ciphertext.1));
    // Unchecked unsafe UTF8 symbols conversion from byte string
    debug!(target: "darkproto", "Ciphertext (unchecked): {:?},{:?},{}\n", &ciphertext.0, &ciphertext.2, unsafe { String::from_utf8_unchecked(ciphertext.1.clone())});

    // Create a named alias for each actor in a swarm
    let alice = swarm.get_actor(0).id;
    let bob = swarm.get_actor(1).id;
    let clara = swarm.get_actor(2).id;
    debug!(target: "darkproto", "Aliases for actors has been created\n");

    // Send all nodes swarm metadata, including ContentID, Node IDs, SK & PK shares (key fragments),
    // ciphertext (for future participation in decryption meeting & redundancy), to nodes swarm;
    // All data will be encrypted for transfer by carrier node PK + servicing node SK during ECDH session;
    send_msg(swarm.get_actor(alice), ciphertext.clone());
    send_msg(swarm.get_actor(bob), ciphertext.clone());
    send_msg(swarm.get_actor(clara), ciphertext);
    debug!(target: "darkproto", "Actors metadata and ciphertext has been sent in a message to the each corresponding node in a swarm\n");

    // Store metadata for future decryption meeting on a servicing node:
    // Content ID,
    // Node IDs,
    // PK_set,
    // ciphertext
    debug!(target: "darkproto", "Metadata (content ID, node IDs, PK set, ciphertext) has been stored on a servicing node\n");

    // Distributed file storage, for encrypted data, stores:
    // Content ID,
    // Encrypted file (content/data) of digital asset,
    // Master Public Key (for cryptosystem configuration and encryption),
    // CP_PK (to verify the content origin, integrity and content recovery possibility),
    // CP_Signature (to verify the content origin, integrity),
    // Node_PK (to verify content origin from authorized source and content recovery possibility),
    // Node_Signature (to verify content origin from authorized source)
    debug!(target: "darkproto", "Encrypted file (content/data) of digital asset and content metadata (content ID, master public key of a swarm, content producer PK and signature, servicing node PK and signature) has been stored in distributed file storage\n");

    // Init decryption meeting
    debug!(target: "darkproto", "Decryption meeting initialization...\n");

    // Retrieve swarm metadata:
    // Content ID,
    // Node IDs,
    // PK_set,
    // Ciphertext;
    debug!(target: "darkproto", "Retrieving swarm's metadata (content ID, node IDs, PK set, ciphertext)\n");

    // Start a decryption meeting. At the meeting, each actor contributes their secret/public key shares (key fragments)
    // for the decryption process to decrypt the ciphertext that they each received;
    debug!(target: "darkproto", "Starting decryption meeting...\n");
    let mut meeting = swarm.start_decryption_meeting();

    // Request p2p swarm for SK/PK shares from actors/nodes and generate decryption shares/fragments from them;
    // Verify/validate decryption shares using PK shares;
    // Collect decryption shares into decryption meeting data structure;
    debug!(target: "darkproto", "Requiesting p2p swarm for SK/PK shares from actors/nodes...\n");

    // Alice is the first actor to arrive at the meeting, she provides her decryption share.
    // According to threshold, one actor alone cannot decrypt the ciphertext, decryption fails.
    debug!(target: "darkproto", "Alice joined meeting. Receiving shares from first actor...\n");
    meeting.accept_decryption_share(swarm.get_actor(alice));
    debug!(target: "darkproto", "Message decryption attempt...\n");
    assert!(meeting.decrypt_message().is_err());
    debug!(target: "darkproto", "Message decryption fails.\n");

    // Bob joins the meeting and provides his decryption share. Alice and Bob are now collaborating to decrypt the ciphertext,
    // they succeed because the threshold requires two or more actors for decryption.
    debug!(target: "darkproto", "Bob joined meeting. Receiving shares from second actor...\n");
    meeting.accept_decryption_share(swarm.get_actor(bob));
    debug!(target: "darkproto", "Message decryption attempt...\n");
    let mut result = meeting.decrypt_message();
    assert!(result.is_ok());
    assert_eq!(msg, result.as_ref().unwrap());
    debug!(target: "darkproto", "Message decryption successful.\n");
    info!(target: "darkproto", "{}\n", String::from_utf8_lossy(result.as_ref().unwrap()));

    // Clara joins the meeting and provides her decryption share.
    // We already are able to decrypt the ciphertext with 2 actors,
    // but let's show that we can do this with 3 actors (with a full swarm) as well.
    debug!(target: "darkproto", "Clara joined meeting. Receiving shares from third actor...\n");
    meeting.accept_decryption_share(swarm.get_actor(clara));
    debug!(target: "darkproto", "Message decryption attempt...\n");
    result = meeting.decrypt_message();
    assert!(result.is_ok());
    assert_eq!(msg, result.as_ref().unwrap());
    debug!(target: "darkproto", "Message decryption successful.\n");
    info!(target: "darkproto", "{}\n", String::from_utf8_lossy(result.as_ref().unwrap()));

    debug!(target: "darkproto", "Decryption meeting resulted data: {:?}\n", meeting);

    // Take content file from distributed storage;
    // Re-encrypt content file using Node_SS = CP_SS for decryption and Node_ReEnc_SS = CC_SS for re-encryption
    //  with immediate streaming to content consumer (and content decryption on consumer end);
    debug!(target: "darkproto", "Taking content file from distributed storage...\n");
    debug!(target: "darkproto", "Gradual re-encryptiion (decryptiion with content secret key and encryptiion with consumer secret key) of content file with streaming to consumer...\n");
    info!(target: "darkproto", "Successfully finished!!!\n");
}
